use crate::format::FileFormat;

pub(crate) const LINGUIST_144: FileFormat = FileFormat {
    id: 144,
    puid: "linguist/144",
    name: "HCL",
    extensions: &["hcl", "nomad", "tf", "tfvars", "workflow"],
    media_types: &["text/x-ruby"],
    internal_signatures: &[],
    related_formats: &[],
};
