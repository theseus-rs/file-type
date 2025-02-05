use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113274726: FileFormat = FileFormat {
    id: 113_274_726,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Deluxe Label",
    extensions: &["pda"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
