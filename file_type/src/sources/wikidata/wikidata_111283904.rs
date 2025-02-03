use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111283904: FileFormat = FileFormat {
    id: 111_283_904,
    source_type: SourceType::Wikidata,
    name: "ITU G.722 6-bit (48 kbps) ADPCM format data",
    extensions: &["g722-6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
