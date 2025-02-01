use crate::format::FileFormat;

pub(crate) const LINGUIST_281: FileFormat = FileFormat {
    id: 281,
    puid: "linguist/281",
    name: "Pascal",
    extensions: &["dfm", "dpr", "inc", "lpr", "pas", "pascal", "pp"],
    media_types: &["text/x-pascal"],
    internal_signatures: &[],
    related_formats: &[],
};
