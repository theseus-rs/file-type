use crate::format::FileFormat;

pub(crate) const LINGUIST_263: FileFormat = FileFormat {
    id: 263,
    puid: "linguist/263",
    name: "OpenCL",
    extensions: &["cl", "opencl"],
    media_types: &["text/x-csrc"],
    internal_signatures: &[],
    related_formats: &[],
};
