use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129571634: FileFormat = FileFormat {
    id: 129_571_634,
    puid: "wikidata/129571634",
    name: "Hy source code file",
    extensions: &["hy", "hy"],
    media_types: &["application/x-hy", "text/x-hy"],
    internal_signatures: &[],
    related_formats: &[],
};
