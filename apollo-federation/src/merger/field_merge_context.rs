use std::collections::HashMap;

use crate::merger::merge::Sources;

#[derive(Debug, Default, Clone)]
pub(crate) struct FieldMergeContextProperties {
    pub(crate) used_overridden: bool,
    pub(crate) unused_overridden: bool,
    pub(crate) override_with_unknown_target: bool,
    pub(crate) override_label: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct FieldMergeContext {
    props: HashMap<usize, FieldMergeContextProperties>,
}

impl FieldMergeContext {
    pub(crate) fn new<T>(sources: &Sources<T>) -> Self {
        let mut props = HashMap::new();
        for (&i, _) in sources.iter() {
            props.insert(
                i,
                FieldMergeContextProperties {
                    used_overridden: false,
                    unused_overridden: false,
                    override_with_unknown_target: false,
                    override_label: None,
                },
            );
        }
        Self { props }
    }

    pub(crate) fn is_used_overridden(&self, idx: usize) -> bool {
        self.props.get(&idx).map(|p| p.used_overridden).unwrap_or(false)
    }

    pub(crate) fn is_unused_overridden(&self, idx: usize) -> bool {
        self.props
            .get(&idx)
            .map(|p| p.unused_overridden)
            .unwrap_or(false)
    }

    pub(crate) fn has_override_with_unknown_target(&self, idx: usize) -> bool {
        self.props
            .get(&idx)
            .map(|p| p.override_with_unknown_target)
            .unwrap_or(false)
    }

    pub(crate) fn override_label(&self, idx: usize) -> Option<&str> {
        self.props.get(&idx).and_then(|p| p.override_label.as_deref())
    }

    pub(crate) fn set_used_overridden(&mut self, idx: usize) {
        if let Some(p) = self.props.get_mut(&idx) {
            p.used_overridden = true;
        }
    }

    pub(crate) fn set_unused_overridden(&mut self, idx: usize) {
        if let Some(p) = self.props.get_mut(&idx) {
            p.unused_overridden = true;
        }
    }

    pub(crate) fn set_override_with_unknown_target(&mut self, idx: usize) {
        if let Some(p) = self.props.get_mut(&idx) {
            p.override_with_unknown_target = true;
        }
    }

    pub(crate) fn set_override_label(&mut self, idx: usize, label: String) {
        if let Some(p) = self.props.get_mut(&idx) {
            p.override_label = Some(label);
        }
    }

    pub(crate) fn some<F>(&self, mut predicate: F) -> bool
    where
        F: FnMut(&FieldMergeContextProperties, usize) -> bool,
    {
        self.props
            .iter()
            .any(|(&idx, props)| predicate(props, idx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_merge_context_properties() {
        // Set up sources for 2 subgraphs
        let sources: Sources<()> = [(0, Some(())), (1, None)].into_iter().collect();

        let mut ctx = FieldMergeContext::new(&sources);
        // Defaults
        assert!(!ctx.is_used_overridden(0));
        assert!(!ctx.is_unused_overridden(1));
        assert!(ctx.override_label(1).is_none());

        // Update properties
        ctx.set_used_overridden(0);
        ctx.set_unused_overridden(1);
        ctx.set_override_with_unknown_target(1);
        ctx.set_override_label(1, "label".to_string());

        // Verify getters
        assert!(ctx.is_used_overridden(0));
        assert!(ctx.is_unused_overridden(1));
        assert!(ctx.has_override_with_unknown_target(1));
        assert_eq!(ctx.override_label(1), Some("label"));

        // Predicate query
        assert!(ctx.some(|p, idx| idx == 0 && p.used_overridden));
    }
}

