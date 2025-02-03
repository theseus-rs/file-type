use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110039981: FileFormat = FileFormat {
    id: 110_039_981,
    source_type: SourceType::Wikidata,
    name: "Phantom CINE Compressed Video File",
    extensions: &["cci"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
