use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111283900: FileFormat = FileFormat {
    id: 111_283_900,
    source_type: SourceType::Wikidata,
    name: "G.721 4-bit (32 kbps) ADPCM format data",
    extensions: &["g721"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
