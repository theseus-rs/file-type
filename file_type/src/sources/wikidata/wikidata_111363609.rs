use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111363609: FileFormat = FileFormat {
    id: 111_363_609,
    source_type: SourceType::Wikidata,
    name: "Westacott WinRanX instrument file",
    extensions: &["wrf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
