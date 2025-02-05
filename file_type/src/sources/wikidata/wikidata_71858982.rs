use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71858982: FileFormat = FileFormat {
    id: 71_858_982,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version 10",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
