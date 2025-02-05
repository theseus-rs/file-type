use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111594330: FileFormat = FileFormat {
    id: 111_594_330,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Library, version 4",
    extensions: &["indl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
