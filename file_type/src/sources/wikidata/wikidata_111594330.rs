use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111594330: FileFormat = FileFormat {
    id: 111_594_330,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Library, version 4",
    extensions: &["indl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
