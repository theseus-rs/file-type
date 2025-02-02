use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_72000076: FileFormat = FileFormat {
    id: 72_000_076,
    source_type: SourceType::Wikidata,
    name: "File Express Index Header",
    extensions: &["ixh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
