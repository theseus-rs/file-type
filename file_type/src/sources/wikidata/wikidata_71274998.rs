use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71274998: FileFormat = FileFormat {
    id: 71_274_998,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version 8",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
