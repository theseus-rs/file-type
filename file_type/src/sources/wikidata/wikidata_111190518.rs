use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111190518: FileFormat = FileFormat {
    id: 111_190_518,
    source_type: SourceType::Wikidata,
    name: "Visual Tool Markup Language File",
    extensions: &["vtml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
