use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127515046: FileFormat = FileFormat {
    id: 127_515_046,
    source_type: SourceType::Wikidata,
    name: "Typescript declaration file",
    extensions: &["d.ts"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
