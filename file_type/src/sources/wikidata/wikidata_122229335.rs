use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122229335: FileFormat = FileFormat {
    id: 122_229_335,
    source_type: SourceType::Wikidata,
    name: "WPA-PSK Export Hash",
    extensions: &["wph"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
