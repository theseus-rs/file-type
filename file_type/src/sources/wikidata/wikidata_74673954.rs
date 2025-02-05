use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_74673954: FileFormat = FileFormat {
    id: 74_673_954,
    source_type: SourceType::Wikidata,
    name: "TurboTax return file",
    extensions: &["tax"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
