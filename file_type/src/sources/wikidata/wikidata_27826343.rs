use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27826343: FileFormat = FileFormat {
    id: 27_826_343,
    source_type: SourceType::Wikidata,
    name: "Auxiliary file, AUX.XML variant",
    extensions: &["aux.xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
