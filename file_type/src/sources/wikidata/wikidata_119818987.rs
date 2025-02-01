use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119818987: FileFormat = FileFormat {
    id: 119_818_987,
    puid: "wikidata/119818987",
    name: "Final Draft AV Document",
    extensions: &["xav"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
