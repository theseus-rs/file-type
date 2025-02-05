use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111363609: FileFormat = FileFormat {
    id: 111_363_609,
    source_type: SourceType::Wikidata,
    name: "Westacott WinRanX instrument file",
    extensions: &["wrf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
