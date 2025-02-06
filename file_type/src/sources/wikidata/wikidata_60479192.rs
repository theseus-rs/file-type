use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60479192: FileFormat = FileFormat {
    id: 60_479_192,
    source_type: SourceType::Wikidata,
    name: "Quattro Pro Spreadsheet for Windows",
    extensions: &["wb1"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
