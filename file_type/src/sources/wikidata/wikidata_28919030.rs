use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28919030: FileFormat = FileFormat {
    id: 28_919_030,
    source_type: SourceType::Wikidata,
    name: "AC-3 Compressed Audio (Dolby Digital), Revision A",
    extensions: &["ac3"],
    media_types: &["audio/ac3"],
    signatures: &[],
    related_formats: &[],
};
