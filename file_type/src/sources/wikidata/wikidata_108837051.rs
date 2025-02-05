use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_108837051: FileFormat = FileFormat {
    id: 108_837_051,
    source_type: SourceType::Wikidata,
    name: "Nero UDF CD-ROM Compilation",
    extensions: &["nru"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
