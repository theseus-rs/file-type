use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5531823: FileFormat = FileFormat {
    id: 5_531_823,
    source_type: SourceType::Wikidata,
    name: "General Data Format for Biomedical Signals",
    extensions: &["gdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
