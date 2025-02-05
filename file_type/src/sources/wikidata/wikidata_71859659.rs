use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71859659: FileFormat = FileFormat {
    id: 71_859_659,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version X4",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
