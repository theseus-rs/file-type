use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650311: FileFormat = FileFormat {
    id: 29_650_311,
    source_type: SourceType::Wikidata,
    name: "POV-Ray scene description",
    extensions: &["inc", "pov"],
    media_types: &["model/x-pov", "text/x-povray"],
    internal_signatures: &[],
    related_formats: &[],
};
