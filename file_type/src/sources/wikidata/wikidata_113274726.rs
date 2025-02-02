use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113274726: FileFormat = FileFormat {
    id: 113_274_726,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Deluxe Label",
    extensions: &["pda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
