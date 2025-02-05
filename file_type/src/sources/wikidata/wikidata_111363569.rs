use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111363569: FileFormat = FileFormat {
    id: 111_363_569,
    source_type: SourceType::Wikidata,
    name: "id Software Music Format (700Hz)",
    extensions: &["wlf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
