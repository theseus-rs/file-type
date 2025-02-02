use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27826389: FileFormat = FileFormat {
    id: 27_826_389,
    source_type: SourceType::Wikidata,
    name: "Proxy AUX file, AUX variant",
    extensions: &["aux"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
