use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111283900: FileFormat = FileFormat {
    id: 111_283_900,
    source_type: SourceType::Wikidata,
    name: "G.721 4-bit (32 kbps) ADPCM format data",
    extensions: &["g721"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
