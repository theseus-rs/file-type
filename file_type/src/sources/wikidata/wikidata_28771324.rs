use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28771324: FileFormat = FileFormat {
    id: 28_771_324,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Spreadsheet",
    extensions: &["wps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
