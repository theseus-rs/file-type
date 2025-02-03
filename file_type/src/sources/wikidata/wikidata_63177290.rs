use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63177290: FileFormat = FileFormat {
    id: 63_177_290,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Spreadsheet for Macintosh, version 3",
    extensions: &["wks"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
