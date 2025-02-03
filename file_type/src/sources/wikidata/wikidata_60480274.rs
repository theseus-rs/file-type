use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60480274: FileFormat = FileFormat {
    id: 60_480_274,
    source_type: SourceType::Wikidata,
    name: "Quattro Pro Spreadsheet for Windows, version 6",
    extensions: &["wb2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
