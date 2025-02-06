use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650311: FileFormat = FileFormat {
    id: 29_650_311,
    source_type: SourceType::Wikidata,
    name: "POV-Ray scene description",
    extensions: &["inc", "pov"],
    media_types: &["model/x-pov", "text/x-povray"],
    signatures: &[],
    related_formats: &[],
};
