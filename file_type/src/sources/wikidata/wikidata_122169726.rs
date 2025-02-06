use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122169726: FileFormat = FileFormat {
    id: 122_169_726,
    source_type: SourceType::Wikidata,
    name: "crypt() Password Hash",
    extensions: &["crypt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
