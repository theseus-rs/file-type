use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27826343: FileFormat = FileFormat {
    id: 27_826_343,
    source_type: SourceType::Wikidata,
    name: "Auxiliary file, AUX.XML variant",
    extensions: &["aux.xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
