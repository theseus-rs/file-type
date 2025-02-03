use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_58799889: FileFormat = FileFormat {
    id: 58_799_889,
    source_type: SourceType::Wikidata,
    name: "PowerProject Teamplan",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
