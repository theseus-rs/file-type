use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111262682: FileFormat = FileFormat {
    id: 111_262_682,
    source_type: SourceType::Wikidata,
    name: "Yamaha A3000 sample file",
    extensions: &["a3s"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
