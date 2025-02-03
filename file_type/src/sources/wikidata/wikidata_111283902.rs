use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111283902: FileFormat = FileFormat {
    id: 111_283_902,
    source_type: SourceType::Wikidata,
    name: "ITU G.722 ADPCM format data",
    extensions: &["g722"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
