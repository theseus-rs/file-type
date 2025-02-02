use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60479192: FileFormat = FileFormat {
    id: 60_479_192,
    source_type: SourceType::Wikidata,
    name: "Quattro Pro Spreadsheet for Windows",
    extensions: &["wb1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
