use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111440975: FileFormat = FileFormat {
    id: 111_440_975,
    source_type: SourceType::Wikidata,
    name: "Visual Basic property PAGe file",
    extensions: &["pag"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
