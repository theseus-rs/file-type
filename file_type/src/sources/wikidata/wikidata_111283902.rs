use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111283902: FileFormat = FileFormat {
    id: 111_283_902,
    source_type: SourceType::Wikidata,
    name: "ITU G.722 ADPCM format data",
    extensions: &["g722"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
