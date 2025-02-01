use crate::format::FileFormat;

pub(crate) const LINGUIST_161: FileFormat = FileFormat {
    id: 161,
    puid: "linguist/161",
    name: "IDL",
    extensions: &["dlm", "pro"],
    media_types: &["text/x-idl"],
    internal_signatures: &[],
    related_formats: &[],
};
