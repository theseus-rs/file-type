use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27826390: FileFormat = FileFormat {
    id: 27_826_390,
    source_type: SourceType::Wikidata,
    name: "Proxy AUX file, AUX.XML variant",
    extensions: &["aux.xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
