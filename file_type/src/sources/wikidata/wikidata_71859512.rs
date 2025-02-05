use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71859512: FileFormat = FileFormat {
    id: 71_859_512,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version X3",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
