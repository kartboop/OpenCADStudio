use document::{entities::Region, EntityType, Handle};
use iced::Task;
use kernel::brep::Body;

use super::Message;
use crate::scene::model::solid_model;

impl super::OpenCADStudio {
    pub(super) fn add_region_model(&mut self, mut region: Region, body: Body) -> Handle {
        let i = self.active_tab;
        region.wires = solid_model::edge_wires(&body);
        let Some(document) = crate::scene::convert::acis_export::solid_to_sat(&body) else {
            self.command_line
                .push_error(crate::t!("The region could not be encoded as ACIS.").as_ref());
            return Handle::NULL;
        };
        region.set_sat_document(&document);
        let Some(handle) = self.commit_entity_handle(EntityType::Region(region)) else {
            return Handle::NULL;
        };
        if !self.tabs[i].scene.register_solid_model(handle, body) {
            self.tabs[i].scene.rollback_new_entities(&[handle]);
            return Handle::NULL;
        }
        handle
    }

    pub(super) fn fit_spline(&mut self) -> Task<Message> {
        use document::entities::Spline;
        use document::types::Vector3;

        let i = self.active_tab;
        let found: Option<(Handle, Vec<[f64; 3]>)> = self.tabs[i]
            .scene
            .selected_entities()
            .iter()
            .filter(|(h, _)| !self.tabs[i].scene.is_layer_locked(*h))
            .find_map(|(h, e)| {
                let EntityType::LwPolyline(_) = e else {
                    return None;
                };
                let curve = crate::entities::curve::entity_curve(e)?;
                let kernel::geom2d::Curve::Polyline(polyline) = &curve.curve else {
                    return None;
                };
                Some((
                    *h,
                    polyline
                        .vertices
                        .iter()
                        .map(|vertex| curve.plane.point_at(vertex.position))
                        .collect(),
                ))
            });
        let Some((handle, fit)) = found else {
            self.command_line.push_error(
                crate::t!("SPLINEFIT: select a polyline to fit a spline through.").as_ref(),
            );
            return Task::none();
        };
        if fit.len() < 3 {
            self.command_line
                .push_error(crate::t!("SPLINEFIT: need at least 3 points.").as_ref());
            return Task::none();
        }
        let n = fit.len();
        let m = n - 1;
        let p = |k: usize| glam::DVec3::new(fit[k][0], fit[k][1], fit[k][2]);
        let mut ctrl: Vec<Vector3> = Vec::with_capacity(3 * m + 1);
        ctrl.push(Vector3::new(fit[0][0], fit[0][1], fit[0][2]));
        for seg in 0..m {
            let p0 = p(seg);
            let p1 = p(seg + 1);
            let prev = if seg > 0 { p(seg - 1) } else { p0 };
            let next = if seg + 2 <= m { p(seg + 2) } else { p1 };
            let b1 = p0 + (p1 - prev) / 6.0;
            let b2 = p1 - (next - p0) / 6.0;
            ctrl.push(Vector3::new(b1.x, b1.y, b1.z));
            ctrl.push(Vector3::new(b2.x, b2.y, b2.z));
            ctrl.push(Vector3::new(p1.x, p1.y, p1.z));
        }
        let mut knots: Vec<f64> = vec![0.0; 4];
        for s in 1..m {
            knots.extend_from_slice(&[s as f64, s as f64, s as f64]);
        }
        knots.extend_from_slice(&[m as f64; 4]);
        let mut spl = Spline::new();
        spl.degree = 3;
        spl.control_points = ctrl;
        spl.knots = knots;
        spl.fit_points = fit.iter().map(|q| Vector3::new(q[0], q[1], q[2])).collect();
        self.push_undo_snapshot(i, "SPLINEFIT");
        self.tabs[i].scene.erase_entities(&[handle]);
        self.tabs[i].scene.add_entity(EntityType::Spline(spl));
        self.tabs[i].dirty = true;
        self.refresh_properties();
        self.command_line
            .push_output(crate::tf!("SPLINEFIT: fit a spline through {n} points.").as_ref());
        Task::none()
    }
}
