use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72000076: FileFormat = FileFormat {
    id: 72_000_076,
    source_type: SourceType::Wikidata,
    name: "File Express Index Header",
    extensions: &["ixh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
