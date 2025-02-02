use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_5531823: FileFormat = FileFormat {
    id: 5_531_823,
    source_type: SourceType::Wikidata,
    name: "General Data Format for Biomedical Signals",
    extensions: &["gdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
