use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111262682: FileFormat = FileFormat {
    id: 111_262_682,
    source_type: SourceType::Wikidata,
    name: "Yamaha A3000 sample file",
    extensions: &["a3s"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
