use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60886323: FileFormat = FileFormat {
    id: 60_886_323,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel 2000-2003 Workbook",
    extensions: &["xls", "xlw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
