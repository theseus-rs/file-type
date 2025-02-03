use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60558754: FileFormat = FileFormat {
    id: 60_558_754,
    source_type: SourceType::Wikidata,
    name: "NuFile Exchange Archival Library",
    extensions: &["bxy", "sdk", "shk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
