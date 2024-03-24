use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub(crate) enum ProxyUiStateError {
    MultipleProxyUiPerEntity,
    DuplicateProxyUi,
}

impl std::fmt::Display for ProxyUiStateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str_value: &'static str = self.into();
        write!(f, "{str_value}")
    }
}

impl From<ProxyUiStateError> for &'static str {
    fn from(error: ProxyUiStateError) -> Self {
        error.into()
    }
}

impl From<&ProxyUiStateError> for &'static str {
    fn from(error: &ProxyUiStateError) -> Self {
        match error {
            ProxyUiStateError::MultipleProxyUiPerEntity => "Multiple proxy UI per entity",
            ProxyUiStateError::DuplicateProxyUi => "Duplicate proxy UI",
        }
    }
}

impl From<ProxyUiStateError> for String {
    fn from(error: ProxyUiStateError) -> Self {
        error.into()
    }
}

impl From<&ProxyUiStateError> for String {
    fn from(error: &ProxyUiStateError) -> Self {
        error.into()
    }
}
