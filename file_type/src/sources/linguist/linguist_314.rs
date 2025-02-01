use crate::format::FileFormat;

pub(crate) const LINGUIST_314: FileFormat = FileFormat {
    id: 314,
    puid: "linguist/314",
    name: "RPM Spec",
    extensions: &["spec"],
    media_types: &["text/x-rpm-spec"],
    internal_signatures: &[],
    related_formats: &[],
};
