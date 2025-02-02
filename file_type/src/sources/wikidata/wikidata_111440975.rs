use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111440975: FileFormat = FileFormat {
    id: 111_440_975,
    source_type: SourceType::Wikidata,
    name: "Visual Basic property PAGe file",
    extensions: &["pag"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
