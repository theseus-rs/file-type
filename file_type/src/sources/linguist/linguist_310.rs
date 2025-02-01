use crate::format::FileFormat;

pub(crate) const LINGUIST_310: FileFormat = FileFormat {
    id: 310,
    puid: "linguist/310",
    name: "REALbasic",
    extensions: &["rbbas", "rbfrm", "rbmnu", "rbres", "rbtbar", "rbuistate"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
